/* Reexport of Styled Components with Theme interface */
import * as styledComponents from "styled-components";
import { ThemeInterface } from "./types";

const {
    default: styled,
    css,
    createGlobalStyle,
    keyframes,
    ThemeProvider,
} = styledComponents as styledComponents.ThemedStyledComponentsModule<
    ThemeInterface
>;

export { css, createGlobalStyle, keyframes, ThemeProvider };
export default styled;
