import { DeepPartial } from "utility-types";
import { PkgError } from "./error";

// TODO: Submit PR to update the extension interface
export interface ModernExtension extends ExtensionAPI.IExtension {
    /**
     * Runs before the objects is destroyed either by removing it, switching
     * sheet, opening the selection tool or any other means of removing the
     * extension. Use this to remove bindings and prevent memory-leaks.
     */
    beforeDestroy?(): void;
    /**
     * Runs once when the object is initializing.
     */
    mounted?(elem: HTMLElement[], layout: any): void;
}

export interface QlikProps {}

export interface Layout {
    [key: string]: unknown;
    props?: DeepPartial<QlikProps>;
}

export interface ThemeInterface {}
