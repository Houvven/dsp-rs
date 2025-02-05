plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.rust.android)
    alias(libs.plugins.kotlin.android)
}

android {
    namespace = "com.bw.dsp"
    compileSdk = 35

    defaultConfig {
        minSdk = 24
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
    cargo {
        module = "../../android"
        libname = "dsp-android"
        targets = listOf("arm64", "arm")
        prebuiltToolchains = true
        verbose = true

        features {
            all()
        }
    }
}

dependencies {
    implementation(libs.androidx.annotation.jvm)
    implementation(libs.androidx.core.ktx)
}

tasks.whenTaskAdded {
    if ((name == "javaPreCompileDebug" || name == "javaPreCompileRelease")) {
        dependsOn("cargoBuild")
    }
}
