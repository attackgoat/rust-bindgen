// bindgen-flags: --rustified-enum .* -- -std=c++14

// Generated by C-Reduce, cleaned up and given names for readability.

template <int, typename>
struct HasNonTypeTemplateParam {
    // But doesn't use the non-type template param nor its type param...
};

enum {
    ENUM_VARIANT_1,
    ENUM_VARIANT_2
};

namespace JS {

template <typename T>
using Alias = HasNonTypeTemplateParam<ENUM_VARIANT_1, T>;

template <typename U>
class Base {
    Alias<U> f;
};

class AutoIdVector : Base<int> {};

}
