import React, { ReactElement, useContext } from "react";
import { Button } from "../../components/Button";
import Icon from "../../components/Icon";
import StyledText from "../../components/StyledText";
import { GameScreenMenuContext, GameScreenMenuType } from "./GameScreenMenuContext";
import { WikiArticleLink } from "../../components/WikiArticleLink";
import { GameStateContext } from "./GameStateContext";
import { MobileContext } from "../MobileContext";

export default function GameScreenMenuTab(props: Readonly<{
    helpMenu: WikiArticleLink | null
    close: GameScreenMenuType | false, 
    children: string 
}>): ReactElement {
    const menuController = useContext(GameScreenMenuContext)!;
    const spectator = useContext(GameStateContext)!.clientState.type === "spectator";
    const mobile = useContext(MobileContext)!;

    return <div className="content-tab">
        <div>
            <StyledText>
                {props.children}
            </StyledText>
        </div>

        {props.close && (!spectator || mobile) && <Button className="close"
            onClick={()=>menuController.closeMenu(props.close as GameScreenMenuType)}
        >
            <Icon size="small">close</Icon>
        </Button>}
        {props.helpMenu && !spectator && <Button className="help"
            onClick={()=>{
                menuController.openMenu(GameScreenMenuType.WikiMenu, ()=>{
                    props.helpMenu && GAME_MANAGER.setWikiArticle(props.helpMenu);
                });
            }}
        >
            <Icon size="small">question_mark</Icon>
        </Button>}
    </div>
}