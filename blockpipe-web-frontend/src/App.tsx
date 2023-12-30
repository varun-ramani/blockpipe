import { ActionIcon, Center, Container, Divider, Group, Stack, Text, Title } from "@mantine/core";
import { IconBrandGithub, IconUser } from "@tabler/icons-react";
import BlockpipeEditor from "./editor";
import tutorialSections from './tut.yml';

function TutorialSection(props: { startingCode: string, expectedCode: string, title: string, content: string }) {
    return <>
        <Divider my={20} />

        <Title order={2}>{props.title}</Title>
        <Text>
            {props.content}
        </Text>

        <BlockpipeEditor defaultValue={props.startingCode} expectValue={props.expectedCode} />
    </>
}

export default function App() {
    return (
        <Stack>
            <Container h={"100vh"}>
                <Center h={"100%"}>
                    <Group>
                        <Stack>
                            <ActionIcon size="lg" variant="light" color="black" onClick={() => location.href = "https://varunramani.com"}>
                                <IconUser />
                            </ActionIcon>
                            <ActionIcon size="lg" variant="light" color="black" onClick={() => location.href = "https://github.com/varun-ramani/blockpipe"}>
                                <IconBrandGithub />
                            </ActionIcon>
                        </Stack>
                        <Title p={10}>|</Title>
                        <Stack>
                            <Title order={2}>An Interactive Tour of BlockPipe</Title>
                            <Title order={4}>A functional language built around the pipe operator</Title>
                            <Text>Scroll to start.</Text>
                        </Stack>
                    </Group>

                </Center>
            </Container>
            <Container py={20}>
                <Title order={4}>Thanks for checking out my language! This page will provide you with an overview of
                    the concepts behind BlockPipe.</Title>

                {Object.values(tutorialSections).map((value: any, idx) => {
                    console.log(value);
                    return <TutorialSection startingCode={value['startingCode']} expectedCode={value['expect']} title={value['title']} content={value['content']} key={idx} />
                })}

            </Container>
        </Stack>

    )
}