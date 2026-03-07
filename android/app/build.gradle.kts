plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android") version "2.0.20" // 使用 2026 年穩定的 Kotlin 版本
}

android {
    // 關鍵：設定唯一的包名空間
    namespace = "io.github.positive_mahjong.tw0hank0"
    compileSdk = 36

    defaultConfig {
        applicationId = "io.github.positive_mahjong.tw0hank0"
        minSdk = 24
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0-alpha"

        // NDK 設定：指定目標架構
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a")
        }

        // 告訴 Gradle 我們的 .so 檔已經由 Python 腳本生成好了，放在 jniLibs 目錄
        // 不需要在此處執行 externalNativeBuild
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            applicationIdSuffix = ".debug"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    sourceSets {
        getByName("main") {
            // 明確指定 jniLibs 的路徑
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    
    // 若需要 Lifecycle 支援可加入
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.8.6")
}