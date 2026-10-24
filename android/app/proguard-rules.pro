# Keep JNI methods called by native code
-keepclasseswithmembernames class * {
    native <methods>;
}

-keep class com.example.linksanitizer.** { *; }
