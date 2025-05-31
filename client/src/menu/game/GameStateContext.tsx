import { createContext, useContext, useEffect, useState } from "react";
import { ChatGroup, GameClient, InsiderGroup, LobbyClientID, ModifierType, PhaseState, PhaseTimes, PlayerIndex, Tag, Verdict } from "../../game/gameState.d";
import { Role, RoleState } from "../../game/roleState.d";
import { ChatMessage } from "../../components/ChatMessage";
import { Grave } from "../../game/graveState";
import { RoleList } from "../../game/roleListState.d";
import ListMap, { ListMapData } from "../../ListMap";
import { ChatFilter } from "./gameScreenContent/ChatMenu";
import { ControllerID, SavedController } from "../../game/abilityInput";
import { defaultPhaseTimes } from "../../game/localStorage";
import { LobbyState } from "../lobby/LobbyContext";
import { WebsocketContext } from "../WebsocketContext";
import { ToClientPacket } from "../../game/packet";

export function useGameStateContext(): GameState{
    const [gameState, setGameState] = useState<GameState>(createGameState());


    type TickData = {count: number, timeDelta: number}
    const [tickData, setTickData] = useState<TickData>({
        count: 0,
        timeDelta: 0
    });

    //start tick
    useEffect(() => {
        const TICK_TIME_DELTA = 1000;
        let tickInterval = setInterval(()=>{
            setTickData(tickData => ({
                count: tickData.count + 1,
                timeDelta: TICK_TIME_DELTA
            }));
        }, TICK_TIME_DELTA);

        return ()=>clearInterval(tickInterval)
    }, []);


    const websocketContext = useContext(WebsocketContext)!;

    useEffect(()=>{
        if(websocketContext.lastMessageRecieved){
            gameStateMessageListener(websocketContext.lastMessageRecieved, gameState);
            //for now
            setGameState(gameState);
        }
    }, [websocketContext.lastMessageRecieved]);

    return gameState;
}

function gameStateMessageListener(packet: ToClientPacket, gameState: GameState){
    
}

export function usePlayerState(): PlayerGameState | undefined {
    const gameState = useContext(GameStateContext);
    if(gameState === undefined){return undefined};
    const { clientState } = gameState

    if (clientState.type === "player") {
        return clientState
    } else {
        return undefined
    }
}
export function usePlayerNames(state?: GameState | LobbyState): string[] | undefined {
    if(state===undefined){
        return undefined;
    }
    if(state.type === "game"){
        return state.players.map((p)=>p.name)
    }
    return state.players.values()
        .filter((c)=>c.clientType.type==="player")
        //thanks typescript very cool
        .map((c)=>c.clientType.type==="player"?c.clientType.name:undefined) as string[]
}
const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }


type GameState = {
    type: "game",
    roomCode: number,
    lobbyName: string,
    
    initialized: boolean,

    myId: number | null,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    phaseState: PhaseState,
    timeLeftMs: number | null,
    dayNumber: number,

    fastForward: boolean,
    
    roleList: RoleList,
    enabledRoles: Role[],
    phaseTimes: PhaseTimes,
    enabledModifiers: ModifierType[],

    ticking: boolean,

    clientState: PlayerGameState | {type: "spectator"},
    host: null | {
        clients: ListMap<LobbyClientID, GameClient>
    },

    missedChatMessages: boolean
}
export default GameState;

export type PlayerGameState = {
    type: "player",

    myIndex: PlayerIndex,
    
    roleState: RoleState,

    will: string,
    notes: string[],
    crossedOutOutlines: number[],
    chatFilter: ChatFilter,
    deathNote: string,
    judgement: Verdict,

    savedControllers: ListMapData<ControllerID, SavedController>,

    fellowInsiders: PlayerIndex[],

    sendChatGroups: ChatGroup[],
    insiderGroups: InsiderGroup[],
    
    missedWhispers: PlayerIndex[]
}

export type Player = {
    name: string,
    index: number,
    numVoted: number,
    alive: boolean,
    roleLabel: Role | null,
    playerTags: Tag[]

    toString(): string
}
export function createGameState(): GameState {
    return {
        type: "game",
        roomCode: 0,
        lobbyName: "",

        initialized: false,

        myId: null,

        chatMessages : [],
        graves: [],
        players: [],
        
        phaseState: {type:"briefing"},
        timeLeftMs: 0,
        dayNumber: 1,

        fastForward: false,
        
        roleList: [],
        enabledRoles: [],
        phaseTimes: defaultPhaseTimes(),
        enabledModifiers: [],

        ticking: true,

        clientState: createPlayerGameState(),
        host: null,

        missedChatMessages: false
    }
}
function createPlayerGameState(): PlayerGameState {
    return {
        type: "player",

        myIndex: 0,
        
        roleState: { type: "detective" },

        savedControllers: [],

        will: "",
        notes: [],
        crossedOutOutlines: [],
        chatFilter: null,
        deathNote: "",
        judgement: "abstain",

        fellowInsiders: [],

        sendChatGroups: [],
        insiderGroups: [],

        missedWhispers: []
    }
}
function createPlayer(name: string, index: number): Player {
    return{
        name: name,
        index: index,
        numVoted: 0,
        alive: true,
        roleLabel: null,
        playerTags: [],

        toString() {
            return ""+(this.index+1)+": " + this.name;
        }
    }
}
