import { Box, Flex, List, ListItem } from "@chakra-ui/react";
import { useAtom } from "jotai";
import {
  githubLanguagesAtom,
  languagesColorAtom,
} from "@shared/model/github/github";

export const View = () => {
  const [githubLanguages] = useAtom(githubLanguagesAtom);
  const [languagesColor] = useAtom(languagesColorAtom);

  return (
    <List flexBasis={"30%"} maxW={"26em"} w={"100%"}>
      {Array.from(githubLanguages.entries())
        .sort((a, b) => b[1] - a[1])
        .map(([language, percent]) => {
          return (
            <ListItem
              key={language}
              display={"flex"}
              gap={"10px"}
              alignItems={"center"}
            >
              <Box
                w={"0.5rem"}
                h={"0.5rem"}
                borderRadius={"50%"}
                background={languagesColor.get(language)}
              />
              <Flex flex={"1 1 auto"} justifyContent={"space-between"}>
                <Box fontWeight={"500"}>{language}</Box>
                <Box fontWeight={"300"}>{percent}%</Box>
              </Flex>
            </ListItem>
          );
        })}
    </List>
  );
};
