import { PhaseTimes } from "../../../game/gameState.d";
import { RoleList } from "../../../game/roleListState.d";
import { Role } from "../../../game/roleState.d";

export type GameModeStorage = {
    format: "v1",
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
}

export type ShareableGameMode = GameModeData & { format: "v1", name: string }