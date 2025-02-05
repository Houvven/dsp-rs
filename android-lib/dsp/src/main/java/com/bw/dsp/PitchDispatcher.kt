package com.bw.dsp

import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder.AudioSource
import androidx.annotation.RequiresPermission
import com.bw.dsp.PitchDetector.PitchDetectedCallback
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.util.Objects

@Suppress("unused")
class PitchDispatcher(
    private var callback: PitchDetectedCallback,
    private val audioRecord: AudioRecord,
) {
    private val coroutineScope = CoroutineScope(Dispatchers.Default)
    private val recordingState get() = audioRecord.recordingState

    fun setPitchDetectedCallback(callback: PitchDetectedCallback?) {
        requireNotNull(callback) { "callback must not be null!!!" }
        this.callback = callback
    }

    fun start() {
        audioRecord.startRecording()
        coroutineScope.launch {
            val buffer = FloatArray(audioRecord.bufferSizeInFrames)
            while (audioRecord.recordingState == AudioRecord.RECORDSTATE_RECORDING) {
                val read = audioRecord.read(buffer, 0, buffer.size, AudioRecord.READ_BLOCKING)
                if (read > 0) {
                    PitchDetector.detect(
                        audioRecord.sampleRate.toDouble(),
                        audioRecord.bufferSizeInFrames,
                        buffer
                    )?.let(callback::onPitchDetected)
                }
            }
        }
    }

    fun stop() {
        audioRecord.stop()
    }

    companion object {
        @JvmStatic
        @RequiresPermission(android.Manifest.permission.RECORD_AUDIO)
        fun create(
            sampleRate: Int,
            bufferSize: Int,
            callback: PitchDetectedCallback
        ): PitchDispatcher {
            val audioRecord = AudioRecord.Builder()
                .setAudioSource(AudioSource.MIC)
                .setAudioFormat(
                    AudioFormat.Builder()
                        .setSampleRate(sampleRate)
                        .setEncoding(AudioFormat.ENCODING_PCM_FLOAT)
                        .setChannelMask(AudioFormat.CHANNEL_IN_MONO)
                        .build()
                )
                .setBufferSizeInBytes(bufferSize)
                .build()

            return PitchDispatcher(
                callback = callback,
                audioRecord = audioRecord
            )
        }
    }
}