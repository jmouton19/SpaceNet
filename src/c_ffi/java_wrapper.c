#include <jni.h>
#include <string.h>
#include "Node.h"
#include "space_net.h"


//Node
JNIEXPORT jlong JNICALL Java_Node_newNode(JNIEnv *env, jobject obj, jstring cluster_name) {
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_node(native_cluster_name);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}

JNIEXPORT jstring JNICALL Java_Node_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_node((void*) nodePtr);
    return (*env)->NewStringUTF(env, zid);
}
JNIEXPORT void JNICALL Java_Node_run(JNIEnv *env, jobject obj, jlong nodePtr) {
    run((void*) nodePtr);
}


//Boot Node
JNIEXPORT jlong JNICALL Java_BootNode_newBoot(JNIEnv *env, jobject obj, jstring cluster_name) {
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_boot(native_cluster_name);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}
JNIEXPORT void JNICALL Java_BootNode_run(JNIEnv *env, jobject obj, jlong nodePtr) {
    run_boot((void*) nodePtr);
}

JNIEXPORT jstring JNICALL Java_BootNode_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_boot((void*) nodePtr);
    return (*env)->NewStringUTF(env, zid);
}
