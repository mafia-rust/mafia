import React, { ReactElement, useMemo } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { getSingleRoleJsonData } from "../../../game/roleState.d";
import { TextDropdownArea } from "../../../components/TextAreaDropdown";
import { GameScreenMenuType } from "../GameScreenMenuContext";
import GameScreenMenuTab from "../GameScreenMenuTab";
import { usePlayerState } from "../GameStateContext";

export function defaultAlibi(): string {
    return DEFAULT_ALIBI;
}
const DEFAULT_ALIBI = "ROLE\nNight 1: \nNight 2:";

export default function WillMenu(): ReactElement {
    const cantChat = usePlayerState()!.sendChatGroups.length === 0;

    const role = usePlayerState()!.roleState.type;

    const alibi = usePlayerState()!.will;
    const notes = usePlayerState()!.notes;
    const deathNote = usePlayerState()!.deathNote;

    const cantPost = useMemo(() => {
        return cantChat
    }, [cantChat])
    
    return <div className="will-menu will-menu-colors">
        <GameScreenMenuTab
            close={GameScreenMenuType.WillMenu}
            helpMenu={"standard/alibi"}
        >
                {translate("menu.will.title")}
        </GameScreenMenuTab>
        <section>
            <TextDropdownArea
                titleString={translate("menu.will.will")}
                defaultOpen={true}
                savedText={alibi}
                cantPost={cantPost}
                onSave={(text) => {
                    GAME_MANAGER.sendSaveWillPacket(text);
                }}
            />
            {(notes.length === 0 ? [""] : notes).map((note: string, i: number) => {
                const title = note.split('\n')[0] || translate("menu.will.notes");
                return <TextDropdownArea
                    key={title + i}
                    titleString={title}
                    savedText={note}
                    cantPost={cantPost}
                    onAdd={() => {
                        if(GAME_MANAGER.state.type === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes.splice(i+1, 0, "");
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                    onSubtract={() => {
                        if(GAME_MANAGER.state.type === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes.splice(i, 1);
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                    onSave={(text) => {
                        if(GAME_MANAGER.state.type === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes[i] = text;
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                />
            })}
            {getSingleRoleJsonData(role).canWriteDeathNote===true ? <TextDropdownArea
                titleString={translate("menu.will.deathNote")}
                savedText={deathNote}
                cantPost={cantPost}
                onSave={(text) => {
                    GAME_MANAGER.sendSaveDeathNotePacket(text);
                }}
            />:null}
        </section>
    </div>
}

