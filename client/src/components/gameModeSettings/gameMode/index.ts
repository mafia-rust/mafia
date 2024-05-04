import { PhaseTimes } from "../../../game/gameState.d";
import { RoleList } from "../../../game/roleListState.d";
import { Role } from "../../../game/roleState.d";

export type GameModeStorage = {
    format: "v0",
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
    disabledRoles: Role[],
}

export type ShareableGameMode = GameModeData & { format: "v0", name: string }