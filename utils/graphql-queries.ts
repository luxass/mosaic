import { gql } from "github-schema";

const REPOSITORY_FRAGMENT = gql`
  #graphql
  fragment RepositoryFragment on Repository {
    name
    isFork
    isArchived
    nameWithOwner
    description
    pushedAt
    url
    # defaultBranchRef {
    #   name
    # }
    # languages(first: 1, orderBy: { field: SIZE, direction: DESC }) {
    #   nodes {
    #     name
    #     color
    #   }
    # }
    # object(expression: "HEAD:.github") {
    #   ... on Tree {
    #     entries {
    #       name
    #       type
    #       path
    #     }
    #   }
    # }
  }
`;

export const PROFILE_QUERY = gql`
  #graphql
  ${REPOSITORY_FRAGMENT}

  query getProfile {
    viewer {
      repositories(
        first: 100
        isFork: false
        privacy: PUBLIC
        orderBy: { field: STARGAZERS, direction: DESC }
      ) {
        totalCount
        nodes {
          ...RepositoryFragment
        }
        pageInfo {
          endCursor
          hasNextPage
        }
      }
      contributions: repositoriesContributedTo(
        privacy: PUBLIC
        first: 100
        contributionTypes: [
          COMMIT
          ISSUE
          PULL_REQUEST
          REPOSITORY
          PULL_REQUEST_REVIEW
        ]
      ) {
        nodes {
          nameWithOwner
        }
      }
    }
  }
`;

export const REPOSITORY_QUERY = gql`
  #graphql
  ${REPOSITORY_FRAGMENT}

  query getRepository($owner: String!, $name: String!) {
    repository(owner: $owner, name: $name) {
      ...RepositoryFragment
    }
  }
`;
