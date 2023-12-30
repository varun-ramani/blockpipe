import { ActionIcon, Box, Group, Stack } from "@mantine/core";
import { Editor } from "@monaco-editor/react";
import { IconCheck, IconPlayerPlay, IconQuestionMark } from "@tabler/icons-react";
import * as blockpipe from 'blockpipe-language';
import { useState } from 'react';

export default function BlockpipeEditor(props: {defaultValue: string, expectValue: string}) {
    const [editorOneValue, setEditorOneValue] = useState(props.defaultValue);
    const [editorTwoValue, setEditorTwoValue] = useState("");

    const handlePlayClick = () => {
        let res = blockpipe.wasm_interpret_from_string(editorOneValue, [], true);
        setEditorTwoValue(res);
    };

    let iconColor = '';
    let icon = null;
    if (editorTwoValue === "") {
        iconColor = "black";
        icon = <IconPlayerPlay />;
    }
    else if (editorTwoValue === props.expectValue) {
        iconColor = "green";
        icon = <IconCheck />;
    }
    else {
        iconColor = "orange";
        icon = <IconQuestionMark />
    }

    

    return (
        <Group style={{ display: 'flex', alignItems: 'center' }} py={20}>
            <Box style={{ flexShrink: 0 }}>
                <ActionIcon size="lg" onClick={handlePlayClick} variant='light' color={iconColor}>
                    {icon}
                </ActionIcon>
            </Box>
            <Stack style={{ flexGrow: 1, flexBasis: '80%' }}>
                <Editor 
                    height="20vh" 
                    value={editorOneValue}
                    onChange={(val) => setEditorOneValue(val || "")} 
                    options={{
                        fontFamily: "Iosevka"
                    }}
                />
                <Editor 
                    height="5vh" 
                    value={editorTwoValue}
                    options={{ readOnly: true, fontFamily: "Iosevka"}} 
                />
            </Stack>
        </Group>
    );
}

