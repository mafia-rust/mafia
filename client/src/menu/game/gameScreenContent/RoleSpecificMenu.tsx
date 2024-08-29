import React, { ReactElement } from "react";
import translate from "../../../game/lang";
import { ContentMenu, ContentTab } from "../GameScreen";
import RoleSpecificSection from "../../../components/RoleSpecific";
import SelectionInformation from "../../../components/SelectionInformation";
import { usePlayerState } from "../../../components/useHooks";

export default function RoleSpecificMenu(): ReactElement {
    const role = usePlayerState(
        playerState => playerState.roleState.type,
        ["yourRoleState"]
    )!;

    return <div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={null}>
            {translate("role."+role+".name")}
        </ContentTab>
        <div>
            <SelectionInformation />
            <RoleSpecificSection/>
        </div>
    </div>
    
}