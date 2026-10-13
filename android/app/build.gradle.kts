import javax.inject.Inject

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

val repositoryRoot: File = rootDir.parentFile
val languageMinSdk: Int = providers.gradleProperty("languageMinSdk").get().toInt()
val languageNdkVersion: String = providers.gradleProperty("languageNdkVersion").get()
val releaseStoreFile: String? = providers.gradleProperty("languageReleaseStoreFile").orNull

android {
    namespace = "ro.zach.language"
    compileSdk = 37
    ndkVersion = languageNdkVersion
    buildToolsVersion = "36.1.0"

    defaultConfig {
        applicationId = "ro.zach.language"
        minSdk = languageMinSdk
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    signingConfigs {
        if (releaseStoreFile != null) {
            create("release") {
                storeFile = file(releaseStoreFile)
                storePassword = providers.gradleProperty("languageReleaseStorePassword").get()
                keyAlias = providers.gradleProperty("languageReleaseKeyAlias").get()
                keyPassword = providers.gradleProperty("languageReleaseKeyPassword").get()
            }
        }
    }

    buildTypes {
        debug {
            buildConfigField("String", "LANGUAGE_ORIGIN", "\"http://127.0.0.1:3000\"")
            buildConfigField("String", "LANGUAGE_REPOSITORY_BASE_URL", "\"http://127.0.0.1:3002\"")
        }

        release {
            buildConfigField("String", "LANGUAGE_ORIGIN", "\"https://language.zach.ro\"")
            buildConfigField("String", "LANGUAGE_REPOSITORY_BASE_URL", "\"https://repository.language.zach.ro\"")
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")

            if (releaseStoreFile != null) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
    }

    buildFeatures {
        compose = true
        buildConfig = true
    }

    sourceSets {
        getByName("main") {
            jniLibs.directories.add(repositoryRoot.resolve("target/uniffi/android/jniLibs").path)
            kotlin.directories.add(repositoryRoot.resolve("target/uniffi/kotlin").path)
        }

        getByName("androidTest") {
            assets.directories.add(repositoryRoot.resolve("data").path)
        }
    }

    testOptions {
        unitTests.isReturnDefaultValues = true
    }
}

kotlin {
    jvmToolchain(21)
}

/* Copies the embedded bundle into a variant's assets, failing when `publish local` has not written it. */
abstract class SyncEmbeddedBundleTask : DefaultTask() {
    @get:InputFiles
    @get:PathSensitive(PathSensitivity.RELATIVE)
    abstract val embeddedBundleDirectory: DirectoryProperty

    @get:OutputDirectory
    abstract val outputDirectory: DirectoryProperty

    @get:Inject
    abstract val fileSystemOperations: FileSystemOperations

    @TaskAction
    fun sync() {
        val courseIndex: File = embeddedBundleDirectory.file("course-index.json").get().asFile

        if (!courseIndex.isFile) {
            throw GradleException("${embeddedBundleDirectory.get().asFile} has no course index; run ./scripts/dev/build-local-repository.sh first")
        }

        fileSystemOperations.sync {
            from(embeddedBundleDirectory)
            into(outputDirectory.dir("embedded_bundle"))
        }
    }
}

val buildRustCore: TaskProvider<Exec> = tasks.register<Exec>("buildRustCore") {
    description = "Builds the Rust core for Android and generates its Kotlin bindings."
    workingDir = repositoryRoot
    commandLine(repositoryRoot.resolve("scripts/android/build-android-core.sh").path)
}

tasks.named("preBuild") {
    dependsOn(buildRustCore)
}

androidComponents {
    onVariants { variant ->
        val syncEmbeddedBundle: TaskProvider<SyncEmbeddedBundleTask> = tasks.register<SyncEmbeddedBundleTask>("syncEmbeddedBundle${variant.name.replaceFirstChar { character -> character.uppercase() }}") {
            embeddedBundleDirectory.set(repositoryRoot.resolve("web/static/embedded"))
        }

        variant.sources.assets?.addGeneratedSourceDirectory(syncEmbeddedBundle, SyncEmbeddedBundleTask::outputDirectory)
    }
}

dependencies {
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.ui)
    implementation(libs.compose.foundation)
    implementation(libs.compose.runtime)
    implementation(libs.compose.material3)
    implementation(libs.compose.ui.tooling.preview)
    implementation(libs.activity.compose)
    implementation(libs.lifecycle.viewmodel.compose)
    implementation(libs.lifecycle.runtime.compose)
    implementation(libs.core.ktx)
    implementation(libs.annotation)
    implementation(libs.kotlinx.coroutines.android)
    implementation(variantOf(libs.jna) { artifactType("aar") })

    testImplementation(libs.junit)
    testImplementation(libs.kotlinx.coroutines.test)

    androidTestImplementation(platform(libs.compose.bom))
    androidTestImplementation(libs.compose.ui.test.junit4)
    androidTestImplementation(libs.androidx.test.runner)
    androidTestImplementation(libs.androidx.test.core)
    androidTestImplementation(libs.androidx.test.ext.junit)
    androidTestImplementation(libs.kotlinx.coroutines.test)
    debugImplementation(libs.compose.ui.test.manifest)
}
