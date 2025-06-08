import { ModifierType, PhaseTimes } from "../../../game/gameState.d";
import { RoleList } from "../../../stateContext/roleListState";
import { Role } from "../../../game/roleState.d";


export type CurrentFormat = "v4";

export type GameModeStorage = {
    format: CurrentFormat,
    gameModes: GameMode[]
};

export type GameMode = {
    name: string,
    // A mapping from number-of-players to game mode data
    data: Record<number, GameModeData>
};

export type GameModeData = {
    roleList: RoleList,
    phaseTimes: PhaseTimes,
    enabledRoles: Role[],
    enabledModifiers: ModifierType[]
}

export type ShareableGameMode = GameModeData & { format: CurrentFormat, name: string }