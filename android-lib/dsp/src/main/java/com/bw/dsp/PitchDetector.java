package com.bw.dsp;

import java.lang.reflect.Field;

@SuppressWarnings("unused")
public class PitchDetector {

    static {
        init();
    }

    private static void init() {
        System.loadLibrary("dsp-rs-android");
    }

    public static native PitchDetectedResult detect(double sampleRate, int bufferSize, float[] audioBuffer);

    @FunctionalInterface
    public interface PitchDetectedCallback {
        void onPitchDetected(PitchDetectedResult result);
    }
}
