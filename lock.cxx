#include <lock.hpp>

Lock::Lock(SDL_mutex *m) noexcept
  : _m(m) { SDL_LockMutex(_m); }

Lock::~Lock() noexcept
  { SDL_UnlockMutex(_m); }

Mutex::Mutex() noexcept
  : _m(SDL_CreateMutex()) { }

Mutex::~Mutex() noexcept
  { SDL_DestroyMutex(_m); }

Lock Mutex::get_lock() noexcept
  { return Lock(_m); }
