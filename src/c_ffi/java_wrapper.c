#include <jni.h>
#include <string.h>
#include "com_example_BootNode.h"
#include "com_example_Node.h"
#include "space_net.h"


//Node
JNIEXPORT jlong JNICALL Java_com_example_Node_newNode(JNIEnv *env, jobject obj, jstring cluster_name, jdouble x, jdouble y) {
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_node(native_cluster_name,x,y);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}

JNIEXPORT jstring JNICALL Java_com_example_Node_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_node((void*) nodePtr);
    return (*env)->NewStringUTF(env, zid);
}
JNIEXPORT void JNICALL Java_com_example_Node_join(JNIEnv *env, jobject obj, jlong nodePtr) {
    join((void*) nodePtr);
}

JNIEXPORT void JNICALL Java_com_example_Node_leaveOnKey(JNIEnv *env, jobject obj, jlong node_ptr, jchar key) {
leave_on_key((void*) node_ptr, (char) key);
}

JNIEXPORT void JNICALL Java_com_example_Node_leave(JNIEnv *env, jobject obj, jlong node_ptr) {
leave((void*) node_ptr);
}

JNIEXPORT jint JNICALL Java_com_example_Node_getStatus(JNIEnv *env, jobject obj, jlong nodePtr) {
    NodeStatus status = get_status((void*) nodePtr);
    return (jint) status;
}

JNIEXPORT jint JNICALL Java_com_example_Node_isNeighbour(JNIEnv *env, jobject obj, jlong nodePtr, jstring zid) {
    const char *native_zid = (*env)->GetStringUTFChars(env, zid, 0);
    jint result = (jint) is_neighbour((void*) nodePtr, native_zid);
    (*env)->ReleaseStringUTFChars(env, zid, native_zid);
    return result;
}

JNIEXPORT jint JNICALL Java_com_example_Node_isInPolygon(JNIEnv *env, jobject obj, jlong nodePtr, jdouble x, jdouble y) {
    jint result = (jint) is_in_polygon((void*) nodePtr, x, y);
    return result;
}


//Boot Node
JNIEXPORT jlong JNICALL Java_com_example_BootNode_newBoot(JNIEnv *env, jobject obj, jstring cluster_name,jboolean centralized_voronoi){
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_boot(native_cluster_name,centralized_voronoi);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}

//JNIEXPORT void JNICALL Java_com_example_BootNode_run(JNIEnv *env, jobject obj, jlong nodePtr) {
//    run_boot((void*) nodePtr);
//}

JNIEXPORT jstring JNICALL Java_com_example_BootNode_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_boot((void*) nodePtr);
    return (*env)->NewStringUTF(env, zid);
}
