#include <stdio.h>
#include <assert.h>

/* copy n bytes from a to b */
void copy_ints(int n, int na, int *a, int nb, int *b) {
    assert(na <= n);
    assert(nb <= n);
    for (int i = 0; i < n; i++) {
        b[i] = a[i];
    }
}

int main() {
    int c[10];
    for (int i = 0; i < 10; i++) {
        c[i] = i;
    }
    copy_ints(3, 3, &1[c], 3, c);
    for (int i = 0; i < 5; i++) {
        printf("%d\n", c[i]);
    }
}
