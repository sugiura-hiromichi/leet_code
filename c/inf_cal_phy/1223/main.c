#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#define EPSILON .000000001
#define DELTA_T .05
#define ECCENTRICITY .5

// report6
double f(double theta, double t) {
  return theta - ECCENTRICITY * sin(theta) - t;
}

// report6
double f_dash(double theta, double t) {
  double right = f(theta + EPSILON, t);
  double left = f(theta, t);
  return (right - left) / EPSILON;
}

// question1
double newton_kepler(double theta, double t) {
  double prev;
  do {
    prev = theta;
    theta = -f(theta, t) / f_dash(theta, t);
  } while (fabs(theta - prev) > EPSILON);

  return theta;
}

int main() {
  // question1
  double t = .25, cur = .5;
  double sol = newton_kepler(cur, t);

  // question2
  // NOTE: T=2π
  double cycle_len = 2 * M_PI;
}
