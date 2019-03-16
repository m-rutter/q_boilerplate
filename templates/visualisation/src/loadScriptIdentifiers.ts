/* loadScriptIdentifiers.ts */
import { checkAppIdentifiers } from "qlik-ts-ident-checker";

export enum Field {}

export enum Variable {}

/**
 * Validates qlik identifers as defined in the load script to check if they match
 * the enums within this project. Logs the result to console
 * @param app Qlik App instance
 */
export async function validate(app: AppAPI.IApp) {
    const result = await checkAppIdentifiers(
        "capabilities",
        app,
        Field,
        Variable,
    );

    if (!result.fields.matching) {
        console.info("Mismatch between qlik and project fields", result.fields);
    }

    if (!result.variables.matching) {
        console.info(
            "Mismatch between qlik and project variables",
            result.variables,
        );
    }

    if (result.variables.matching && result.fields.matching) {
        console.info("Qlik Identifiers validated", result);
    }
}
