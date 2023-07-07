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

JNIEXPORT void JNICALL Java_com_example_Node_free(JNIEnv *env, jobject obj, jlong nodePtr) {
    free_node((void*) nodePtr);
}

JNIEXPORT jstring JNICALL Java_com_example_Node_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_node((void*) nodePtr);
    jstring javaString = (*env)->NewStringUTF(env, zid);
    free_c_string((char*)zid); // Free the memory allocated for the C string
    return javaString;
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

JNIEXPORT jobjectArray JNICALL Java_com_example_Node_getNeighbours(JNIEnv *env, jobject obj, jlong nodePtr) {
    // Call your C function here to get the neighbors as a C string array
    char** neighbors = get_neighbours((void*) nodePtr);

    // Count the number of neighbors in the C string array
    int numNeighbors = 0;
    while (neighbors[numNeighbors] != NULL) {
        numNeighbors++;
    }
    // Create a Java array to store the neighbors
    jclass stringClass = (*env)->FindClass(env, "java/lang/String");
    jobjectArray neighborsArray = (*env)->NewObjectArray(env, numNeighbors, stringClass, NULL);
    // Iterate over the C string array and populate the Java array
    for (int i = 0; i < numNeighbors; i++) {
        jstring neighborString = (*env)->NewStringUTF(env, neighbors[i]);
        (*env)->SetObjectArrayElement(env, neighborsArray, i, neighborString);
        //(*env)->DeleteLocalRef(env, neighborString);
    }
    free_neighbours(neighbors);
    return neighborsArray;
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

JNIEXPORT void JNICALL Java_com_example_Node_sendMessage(JNIEnv *env, jobject obj, jlong nodePtr,jbyteArray buffer,jstring topic) {
    jsize len = (*env)->GetArrayLength(env, buffer);
    jbyte* elements = (*env)->GetByteArrayElements(env, buffer, 0);
    unsigned char* dataPtr = (unsigned char*)elements;

    Buffer cbuffer;
    cbuffer.data = dataPtr;
    cbuffer.len = len;

    // const char *cRecvNode = (*env)->GetStringUTFChars(env, recvNode, 0);
    const char *ctopic = (*env)->GetStringUTFChars(env, topic, 0);
    send_message((void*) nodePtr,cbuffer,ctopic);
}

JNIEXPORT jstring JNICALL Java_com_example_Node_closestNeighbour(JNIEnv *env, jobject obj, jlong nodePtr,jdouble x, jdouble y) {
    const char* neighbour = closest_neighbour((void*) nodePtr,x,y);
    jstring javaString = (*env)->NewStringUTF(env, neighbour);
    free_c_string((char*)neighbour); // Free the memory allocated for the C string
    return javaString;
}








//Boot Node
JNIEXPORT jlong JNICALL Java_com_example_BootNode_newBoot(JNIEnv *env, jobject obj, jstring cluster_name,jboolean centralized_voronoi){
    const char *native_cluster_name = (*env)->GetStringUTFChars(env, cluster_name, 0);
    jlong result = (jlong) new_boot(native_cluster_name,centralized_voronoi);
    (*env)->ReleaseStringUTFChars(env, cluster_name, native_cluster_name);
    return result;
}

JNIEXPORT void JNICALL Java_com_example_BootNode_free(JNIEnv *env, jobject obj, jlong nodePtr) {
    free_boot_node((void*) nodePtr);
}

JNIEXPORT jstring JNICALL Java_com_example_BootNode_getZid(JNIEnv *env, jobject obj, jlong nodePtr) {
    const char* zid = get_zid_boot((void*) nodePtr);
    jstring javaString = (*env)->NewStringUTF(env, zid);
    free_c_string((char*)zid); // Free the memory allocated for the C string
    return javaString;
}







//subscriber
JNIEXPORT jlong JNICALL Java_com_example_NodeSubscriber_newNodeSubscriber(JNIEnv *env, jobject obj, jlong subPtr) {
    jlong result = (jlong) new_subscriber((void*) subPtr);
    return result;
}

JNIEXPORT void JNICALL Java_com_example_NodeSubscriber_free(JNIEnv *env, jobject obj, jlong subPtr) {
    free_subscriber((void*) subPtr);
}

JNIEXPORT void JNICALL Java_com_example_NodeSubscriber_subscribe(JNIEnv *env, jobject obj, jlong subPtr,jstring topic) {
    const char *ctopic = (*env)->GetStringUTFChars(env, topic, 0);

    //int global=(global_sub==JNI_TRUE);
    subscribe((void*) subPtr, ctopic);
}

JNIEXPORT jlong JNICALL Java_com_example_NodeSubscriber_receive(JNIEnv *env, jobject obj, jlong subPtr) {
    jlong result = (jlong) receive((void*) subPtr);
    return result;
}










////////
JNIEXPORT jstring JNICALL Java_com_example_PayloadMessage_getTopic(JNIEnv *env, jobject obj, jlong payloadPtr) {
    const char* topic = get_topic((void*) payloadPtr);
    jstring javaString = (*env)->NewStringUTF(env, topic);
    free_c_string((char*)topic); // Free the memory allocated for the C string
    return javaString;
}

JNIEXPORT jstring JNICALL Java_com_example_PayloadMessage_getSenderId(JNIEnv *env, jobject obj, jlong payloadPtr) {
    const char* sender_id = get_sender_id((void*) payloadPtr);
    jstring javaString = (*env)->NewStringUTF(env, sender_id);
    free_c_string((char*)sender_id); // Free the memory allocated for the C string
    return javaString;
}

JNIEXPORT jbyteArray JNICALL Java_com_example_PayloadMessage_getPayload(JNIEnv *env, jobject obj, jlong payloadPtr) {
    Buffer buffer=get_payload((void*) payloadPtr);
    jbyteArray byteArray = (*env)->NewByteArray(env, buffer.len);
    (*env)->SetByteArrayRegion(env, byteArray, 0, buffer.len, (jbyte*)buffer.data);
    free_buf(buffer);
    return byteArray;
}

JNIEXPORT void JNICALL Java_com_example_PayloadMessage_flush(JNIEnv *env, jobject obj, jlong payloadPtr) {
    free_payload_message((void*) payloadPtr);
}

