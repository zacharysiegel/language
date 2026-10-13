# JNA and the generated UniFFI bindings find their classes and fields by reflection.
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
-keep class ro.zach.language.core.** { *; }
-dontwarn java.awt.**
