import { ActionIcon, Anchor, Center, Container, Divider, Group, Stack, Text, Title } from "@mantine/core";
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
                        <Title p={10}>{`()|{}`}</Title>
                        <Stack>
                            <Title order={2}>An Interactive Tour of BlockPipe</Title>
                            <Title order={4}>A functional language built around the pipe operator</Title>
                            <Text>Scroll to start. Best viewed on desktop.</Text>
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

                <Divider my={20} />
                
                <Stack>
                    <Title order={4}>
                        You made it! That was a whirlwind tour of BlockPipe. What next?
                    </Title>

                    <Text>
                        I'd like to extend my sincere appreciation for your time. I hope that if you made it this far, you found BlockPipe 
                        at least somewhat interesting. A lot of hard work went into this project, and it's definitely one of the coolest things 
                        I've made. 
                    </Text>

                    <Text>
                        Furthermore, if you're curious, there's a lot more you can explore. This webpage is backed by a small component of the BlockPipe interpreter
                        that I compiled to WebAssembly, but the language is actually implemented in Rust and can be executed locally. There's also an 
                        in-development compiler targeting LLVM. 
                    </Text>

                    <Text>
                        If you have some more time, you could take a look at the <Anchor href="https://github.com/varun-ramani/blockpipe">BlockPipe source code</Anchor>. 
                        Or, if you want to learn about some of the other things I've worked on, you could <Anchor href="https://varunramani.com">check out my portfolio.</Anchor> 
                    </Text>

                    <Text>
                        Thanks again, <br />
                        Varun Ramani
                    </Text>
                </Stack>

            </Container>
        </Stack>

    )
}