import { useDark, useToggle } from "@vueuse/core";

export const is_dark = useDark();
export const toggle_dark = useToggle(is_dark)