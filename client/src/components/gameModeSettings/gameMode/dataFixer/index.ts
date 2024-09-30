import { GameModeStorage, ShareableGameMode } from "..";
import { Failure, ParseResult, Success, isFailure } from "../parse"
import initial from "./initial"
import v0 from "./v0"
import v1 from "./v1"
import v2 from "./v2"

/// A version converter from a specified version to the next version
export type VersionConverter = {
    matchGameModeStorage?: (json: NonNullable<any>) => boolean,
    convertGameModeStorage: (json: NonNullable<any>) => ParseResult<NonNullable<any>>,

    matchShareableGameMode?: (json: NonNullable<any>) => boolean,
    convertShareableGameMode: (json: NonNullable<any>) => ParseResult<NonNullable<any>>,
}

type ConverterMap = {
    "GameModeStorage": GameModeStorage,
    "ShareableGameMode": ShareableGameMode
}

const VERSION_CONVERTERS: Record<string, VersionConverter> = { initial, v0, v1, v2 }

/// This converter is the latest version, and acts as a verification step.
export const LATEST_VERSION_STRING = "v2";
const LATEST_VERSION: VersionConverter = VERSION_CONVERTERS[LATEST_VERSION_STRING];

export default function parseFromJson<T extends keyof ConverterMap>(type: T, json: NonNullable<any>): ParseResult<ConverterMap[T]> {
    function isCorrectConverter(json: NonNullable<any>, converter: VersionConverter, version: string) {
        return (typeof json === "object" && !Array.isArray(json) && json.format === version) || 
            (converter[`match${type}`] !== undefined && converter[`match${type}`]!(json))
    }

    let currentJson = json;
    while (true) {
        const converterEntry = Object.entries(VERSION_CONVERTERS)
            // eslint-disable-next-line no-loop-func
            .find(([version, converter]) => isCorrectConverter(currentJson, converter, version));
    
        if (converterEntry === undefined) {
            return Failure("unsupportedFormat", currentJson);
        }

        const converter = converterEntry[1];

        const result = converter[`convert${type}`](currentJson);
        if (isFailure(result))
            return result;

        currentJson = result.value;

        if (converter === LATEST_VERSION) {
            return Success(currentJson);
        }
    }
}