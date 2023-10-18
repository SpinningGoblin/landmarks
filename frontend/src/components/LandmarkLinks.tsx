import { FC, useCallback, useMemo, useState } from "react";
import { LandmarkLink } from "../models/LandmarkLink";
import {
  Button,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  Stack,
  Typography,
} from "@mui/material";
import { LandmarkCard } from "./LandmarkCard";
import {
  useLandmarkLinkTypes,
  useLandmarks,
  useLinkLandmarks,
} from "../hooks/landmarks";

export interface LandmarkLinksProps {
  landmarkLinks: LandmarkLink[];
  landmarkId?: string;
  worldId?: string;
  allowEditing: boolean;
}

export const LandmarkLinks: FC<LandmarkLinksProps> = ({
  worldId,
  landmarkLinks,
  landmarkId,
}) => {
  const { landmarks } = useLandmarks(worldId);
  const { linkTypes } = useLandmarkLinkTypes();
  const [newLinkId, setNewLinkId] = useState("");
  const [newLinkType, setNewLinkType] = useState("");

  const onSuccess = useCallback(() => {
    setNewLinkId("");
    setNewLinkType("");
  }, [setNewLinkId, setNewLinkType]);

  const { addLinks } = useLinkLandmarks(onSuccess, landmarkId);

  const handleOnNewLinkChange = useCallback(
    (event: SelectChangeEvent<string>) => {
      setNewLinkId(event.target.value);
    },
    [setNewLinkId],
  );

  const handleOnNewLinkTypeChange = useCallback(
    (event: SelectChangeEvent<string>) => {
      setNewLinkType(event.target.value);
    },
    [setNewLinkType],
  );

  const possibleLinks = useMemo(() => {
    if (!landmarks) {
      return [];
    }

    return landmarks.filter((landmark) => {
      if (landmark.id === landmarkId) {
        return false;
      }

      return !landmarkLinks.find(
        (link) => link.landmark_metadata.id === landmark.id,
      );
    });
  }, [landmarks, landmarkLinks, landmarkId]);

  const showNewLinks =
    possibleLinks.length > 0 && linkTypes.length > 0 && !addLinks.isPending;

  return (
    <>
      <Typography variant="h4">Linked Landmarks</Typography>
      <Stack spacing={2}>
        {worldId &&
          landmarkLinks.map((landmarkLink) => (
            <LandmarkCard
              key={landmarkLink.landmark_metadata.id}
              landmark={landmarkLink.landmark_metadata}
              worldId={worldId}
            />
          ))}
        {addLinks.isPending && (
          <Typography variant="subtitle1">Saving links...</Typography>
        )}
        {showNewLinks && (
          <>
            <FormControl>
              <InputLabel>New Link</InputLabel>
              <Select
                label="New Link"
                value={newLinkId}
                onChange={handleOnNewLinkChange}
                style={{ minWidth: "20em" }}
              >
                {possibleLinks.map((landmark) => (
                  <MenuItem key={landmark.id} value={landmark.id}>
                    <Typography variant="subtitle1">{landmark.name}</Typography>
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            <FormControl>
              <InputLabel>Link Type</InputLabel>
              <Select
                label="Link Type"
                value={newLinkType}
                onChange={handleOnNewLinkTypeChange}
                style={{ minWidth: "20em" }}
              >
                {linkTypes.map((linkType) => (
                  <MenuItem key={linkType} value={linkType}>
                    <Typography variant="subtitle1">{linkType}</Typography>
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            <Button
              onClick={() => {
                if (!newLinkId) {
                  return;
                }
                addLinks.mutate({
                  landmarkId: newLinkId,
                  linkType: newLinkType,
                });
              }}
            >
              + Add Link
            </Button>
          </>
        )}
      </Stack>
    </>
  );
};
