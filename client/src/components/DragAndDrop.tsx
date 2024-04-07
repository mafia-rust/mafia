import React, { useState } from "react";
import "./dragAndDrop.css"


// From: https://github.com/atlassian/react-beautiful-dnd/issues/316#issuecomment-1860490084
export function DragAndDrop<T>(props: { 
    items: T[], 
    render: (item: T, index: number) => React.ReactNode, 
    onDragEnd: (newItems: T[]) => void }
): React.ReactElement {
    const [temporaryItems, setTemporaryItems] = useState<T[] | null>(null);
    const [draggedItem, setDraggedItem] = useState<T | null>(null);

    const renderedItems = temporaryItems ?? props.items;

    return <>
        {renderedItems.map((item, index) => <div
            key={index}
            className={"draggable" + (item === draggedItem ? " dragged" : "")}
            draggable
            onDragStart={() => setDraggedItem(item)}
            onDragOver={(e) => {
                e.preventDefault();
                if (draggedItem === null || draggedItem === item) {
                    return;
                }
                const currentIndex = renderedItems.indexOf(draggedItem);
                const targetIndex = renderedItems.indexOf(item);
                
                if (currentIndex !== -1 && targetIndex !== -1) {
                    const newItems = [...renderedItems];
                    newItems.splice(currentIndex, 1);
                    newItems.splice(targetIndex, 0, draggedItem);
                    setTemporaryItems(newItems);
                }
            }}
            onDragEnd={() => {
                props.onDragEnd(renderedItems);
                setDraggedItem(null);
                setTemporaryItems(null);
            }}
        >
            {props.render(item, index)}
        </div>)}
    </>
}