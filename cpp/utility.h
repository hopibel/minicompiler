// Utility stuff

#ifndef __UTILITY__
#define __UTILITY__

// Haskell's Maybe
template<typename T>
class Optional {
  bool b;
  T val;
public:
  Optional() : b(false) {}
  Optional(T v) : val(v), b(true) {}
  bool isJust() { return b; }
  bool isNothing() { return !b; }
  T fromJust() { return val; }
};

template<typename T>
Optional<T> nothing() {
  return Optional<T>();
}

template<typename T>
Optional<T> just(T v) {
  return Optional<T>(v);
}


#endif // __UTILITY__
