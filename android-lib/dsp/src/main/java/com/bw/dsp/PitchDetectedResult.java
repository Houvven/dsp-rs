package com.bw.dsp;

import androidx.annotation.Keep;

@Keep
@SuppressWarnings("unused")
public class PitchDetectedResult {

    public float pitch;
    public float probability;
    public boolean pitched;

    public PitchDetectedResult(float pitch, float probability, boolean pitched) {
        this.pitch = pitch;
        this.probability = probability;
        this.pitched = pitched;
    }
}