/* sample.h — a fixture for AWARE sidecar header parsing tests. */

#ifndef SAMPLE_H
#define SAMPLE_H

#ifdef __cplusplus
extern "C" {
#endif

/** Add two integers. */
int sample_add(int a, int b);

/** Multiply two integers. */
int sample_mul(int a, int b);

/** Print a greeting. */
void sample_greet(const char* name);

#ifdef __cplusplus
}
#endif

#endif /* SAMPLE_H */
