#ifndef DIRECTION_HPP
# define DIRECTION_HPP 1
enum direction_t { LE = 0, RI, UP, DO };

inline direction_t against_direction(direction_t d) noexcept {
  return static_cast<direction_t>(1 + d - ((d & 1) << 1));
}

inline void direction2xyd(direction_t d, int& x, int& y) noexcept {
  *((d >> 1) ? &y : &x) += ((d & 1) << 1) - 1;
}
#endif
