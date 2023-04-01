import {
  Box,
  IconButton,
  Input,
  InputGroup,
  InputRightElement,
  useToast,
} from "@chakra-ui/react";
import { ChangeEvent, KeyboardEvent, useRef, useState } from "react";
import { draw } from "@shared/pkg";
import { fetchByUrl } from "@shared/utils/fetchByUrl";
import { Search2Icon } from "@chakra-ui/icons";
import { githubLanguagesAtom, isLoadingAtom } from "@shared/model/github";
import { useAtom } from "jotai";
import { languagesColorAtom } from "@shared/model/github/github";

export const View = () => {
  const toast = useToast();
  const [, setGithubLanguages] = useAtom(githubLanguagesAtom);
  const [, setLanguagesColor] = useAtom(languagesColorAtom);
  const [, setIsLoading] = useAtom(isLoadingAtom);
  const [input, setInput] = useState("");
  const prevFetch = useRef<{ author: string; name: string }>();

  const handleFetchGithub = async () => {
    const repositoryUrl = input.split("/");
    const repositoryAuthor = repositoryUrl.at(-2);
    const repositoryName = repositoryUrl.at(-1);

    if (
      prevFetch.current?.author === repositoryAuthor &&
      prevFetch.current?.name === repositoryName
    )
      return;

    try {
      setIsLoading(true);
      const repositoryResponse = await fetch(
        `https://api.github.com/repos/${repositoryAuthor}/${repositoryName}`.trim()
      );
      const repository = await repositoryResponse.json();
      const { languages_url, message } = repository;

      if (!languages_url) {
        throw new Error(message);
      }

      const requests = [languages_url].map((url) => fetchByUrl(url));
      const [languages] = await Promise.all(requests);

      const [formattedLanguages, languagesColors] = await draw(
        new Map(Object.entries(languages))
      );
      setGithubLanguages(formattedLanguages);
      setLanguagesColor(languagesColors);
    } catch (e) {
      console.error(e);
      toast({
        title: "Error",
        // @ts-ignore
        description: e.message || e,
        status: "error",
        duration: 2000,
        isClosable: true,
      });
    } finally {
      prevFetch.current = {
        name: repositoryName as string,
        author: repositoryAuthor as string,
      };
      setIsLoading(false);
    }
  };

  const handleOnChange = (e: ChangeEvent<HTMLInputElement>) => {
    setInput(e.currentTarget.value);
  };

  const handleOnKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleFetchGithub();
    }
  };

  return (
    <Box as={"header"} display={"flex"} justifyContent={"center"} pt={"30px"}>
      <InputGroup>
        <Input
          onKeyDown={handleOnKeyDown}
          value={input}
          onChange={handleOnChange}
          placeholder={"Enter the github repository link"}
          paddingRight={"50px"}
        />
        <InputRightElement right={"1px"}>
          <IconButton
            h={"38px"}
            aria-label={"search button"}
            onClick={handleFetchGithub}
            icon={<Search2Icon />}
          />
        </InputRightElement>
      </InputGroup>
    </Box>
  );
};
