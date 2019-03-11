import * as React from "react";
import * as qlik from "qlik";

import { useGenericObject } from "../hooks/useGenericObject";
import styled, { ThemeProvider } from "../styled-components";
import { ThemeInterface } from '../types'

type LayoutType = {
    selectedCount: number;
};

const ContainerDiv = styled.div`
    height: calc(100% - 0.5rem);
    width: calc(100% - 0.5rem);
    margin: 0.25rem;
    overflow-y: auto;
    box-sizing: border-box;
    padding: 0.5rem;
`;

const theme: ThemeInterface = {};

const def = {
    selectedCount: {
        qValueExpression: `=1`,
    },
}

export function Root() {

    const qlikState = useGenericObject<LayoutType>(qlik, def);

    if (qlikState.layout) {
        console.log(qlikState.layout.selectedCount)
    }

    return (
        <ThemeProvider theme={theme}>
            <ContainerDiv>Hello Qlik!</ContainerDiv>
        </ThemeProvider>
    );
}
