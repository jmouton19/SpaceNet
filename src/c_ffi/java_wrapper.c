#include <jni.h>
#include <stdio.h>
#include <string.h>
#include "com_example_BootNode.h"
#include "com_example_Node.h"
#include "space_net.h"


//Node
JNIEXPORT jlong JNICALL Java_com_example_Node_newNode(JNIEnv *env, jobject obj, jstring cluster_name) {
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_node(native_cluster_name);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}

JNIEXPORT jstring JNICALL Java_com_example_Node_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_node((void*) nodePtr);
    return (*env)->NewStringUTF(env, zid);
}
JNIEXPORT void JNICALL Java_com_example_Node_join(JNIEnv *env, jobject obj, jlong nodePtr, jdouble x, jdouble y) {
    join((void*) nodePtr,x,y);
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

JNIEXPORT jstring JNICALL Java_com_example_Node_closestNeighbour(JNIEnv *env, jobject obj, jlong nodePtr,jdouble x, jdouble y) {
    const char* zid = closest_neighbour((void*) nodePtr,x,y);
    return (*env)->NewStringUTF(env, zid);
}

JNIEXPORT void JNICALL Java_com_example_Node_sendMessage(JNIEnv *env, jobject obj, jlong nodePtr,jbyteArray buffer,jstring recvNode,jstring topic) {
    jsize len = (*env)->GetArrayLength(env, buffer);
    jbyte* elements = (*env)->GetByteArrayElements(env, buffer, 0);
    unsigned char* dataPtr = (unsigned char*)elements;
    printf("Printing from native function SEND_MESSAGE\n");

    Buffer cbuffer;
    cbuffer.data = dataPtr;
    cbuffer.len = len;
    printf("%02x\n", cbuffer.data[0]);
    printf("%zu\n", cbuffer.len);


    const char *cRecvNode = (*env)->GetStringUTFChars(env, recvNode, 0);
    const char *ctopic = (*env)->GetStringUTFChars(env, topic, 0);
    printf("Topic: %s\n", ctopic);

    send_message((void*) nodePtr,cbuffer,cRecvNode,ctopic);
}

//subscriber
JNIEXPORT jlong JNICALL Java_com_example_NodeSubscriber_newNodeSubscriber(JNIEnv *env, jobject obj, jlong nodePtr) {
    jlong result = (jlong) new_subscriber((void*) nodePtr);
    return result;
}
