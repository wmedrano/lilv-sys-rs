#!/bin/bash
# Script used to regenerate bindgen.rs
echo "use lv2_raw::*;type LV2_URID_Map = LV2UridMap; type LV2_Feature = LV2Feature;type LV2_Handle = LV2Handle; type LV2_Descriptor = LV2Descriptor; type LV2_URID_Unmap = ::std::os::raw::c_void;" >/tmp/lilv-import-lv2.rs
bindgen /usr/include/lilv-0/lilv/lilv.h --whitelist-type "Lilv.*" --blacklist-type "_*LV2.*" --whitelist-function "lilv_.*" --whitelist-var "LILV.*" >/tmp/lilv-bindgen.rs
cat /tmp/lilv-import-lv2.rs /tmp/lilv-bindgen.rs >./src/bindgen.rs
