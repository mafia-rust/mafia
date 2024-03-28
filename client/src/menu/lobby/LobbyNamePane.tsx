import React, { useEffect } from "react";
import { ReactElement } from "react";
import GAME_MANAGER from "../..";
import translate from "../../game/lang";



export default function LobbyNamePane(): ReactElement {


    const spectatorDep = 
        GAME_MANAGER.state.stateType === "lobby" &&
        GAME_MANAGER.state.myId !== null &&
        GAME_MANAGER.state.players.get(GAME_MANAGER.state.myId)?.clientType.type === "spectator";

    
    const [isSpectator, setIsSpectator] = React.useState(spectatorDep);

    useEffect(()=>{
        setIsSpectator(spectatorDep)
    }, [spectatorDep]);



    return <section className="selector-section" style={{backgroundColor: "var(--tab-color)"}}>
        {!isSpectator && <NameSelector/>}
        {isSpectator && <button
            onClick={()=>{GAME_MANAGER.sendSetSpectatorPacket(false)}}
        >{translate("switchToPlayer")}</button>}
    </section>
}

function NameSelector(): ReactElement {

    const [enteredName, setEnteredName] = React.useState("");

    return <>
        <div className="lobby-name">
            <section><h2>{GAME_MANAGER.getMyName() ?? ""}</h2></section>
            <button
                onClick={()=>{GAME_MANAGER.sendSetSpectatorPacket(true)}}
            >{translate("switchToSpectator")}</button>
        </div>
        <div className="name-box">
            <input type="text" value={enteredName}
                onChange={(e)=>{setEnteredName(e.target.value)}}
                placeholder={translate("menu.lobby.field.namePlaceholder")}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        GAME_MANAGER.sendSetNamePacket(enteredName);
                }}
            />
            <button onClick={()=>{
                GAME_MANAGER.sendSetNamePacket(enteredName)
            }}>{translate("menu.lobby.button.setName")}</button>
        </div>
    </>
}