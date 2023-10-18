import { FC, useCallback, useState } from "react";
import { useParams } from "react-router-dom";
import {
  useAddFarm,
  useAddTag,
  useLandmark,
  useLandmarks,
  useRemoveFarm,
  useRemoveTag,
} from "../hooks/landmarks";
import {
  Button,
  Chip,
  Container,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { Coordinate } from "../components/Coordinate";
import { Dimension } from "../components/Dimension";
import { LandmarkBiomes } from "../components/LandmarkBiomes";
import { LandmarkLinks } from "../components/LandmarkLinks";

export const Landmark: FC = () => {
  const { worldId, landmarkId } = useParams();
  const [newFarm, setNewFarm] = useState("");
  const addFarmCallback = useCallback(() => {
    setNewFarm("");
  }, [setNewFarm]);

  const [newTag, setNewTag] = useState("");
  const addTagCallback = useCallback(() => {
    setNewTag("");
  }, [setNewTag]);

  const { landmarks } = useLandmarks(worldId);
  const { landmark, isLoading } = useLandmark(landmarkId);
  const { removeFarm } = useRemoveFarm(landmarkId);
  const { addFarm } = useAddFarm(addFarmCallback, landmarkId);
  const { removeTag } = useRemoveTag(landmarkId);
  const { addTag } = useAddTag(addTagCallback, landmarkId);

  const farmsSaving = removeFarm.isPending || addFarm.isPending;
  const tagsSaving = removeTag.isPending || addTag.isPending;

  console.log((landmarks ?? []).filter((l) => l.id !== landmarkId));

  return (
    <Container>
      {isLoading && <Typography>Loading ...</Typography>}
      {!isLoading && landmark && (
        <Stack spacing={4}>
          <Typography variant="h2">{landmark.metadata.name}</Typography>
          <Coordinate coordinate={landmark.metadata.coordinate} />
          <Dimension dimension={landmark.dimension} />
          <Typography variant="body2" textTransform="capitalize">
            Notes: {landmark.metadata.notes}
          </Typography>
          <LandmarkBiomes
            allowEditing
            landmarkBiomes={landmark.biomes}
            landmarkId={landmarkId}
          />
          {tagsSaving && (
            <Typography variant="subtitle1">Saving Tags</Typography>
          )}
          {!tagsSaving && (
            <Stack direction="row" spacing={1} alignItems="center">
              <Typography variant="subtitle1">Tags</Typography>
              <Stack direction="row" spacing={0.5}>
                {(landmark.tags ?? []).map((tag, index) => (
                  <Chip
                    key={index}
                    label={tag}
                    onDelete={() => removeTag.mutate(tag)}
                  />
                ))}
                <TextField
                  label="Add Tag"
                  onChange={(event) => setNewTag(event.currentTarget.value)}
                  size="small"
                  value={newTag}
                />
                <Button
                  onClick={() => {
                    if (!newTag) {
                      return;
                    }
                    addTag.mutate(newTag);
                  }}
                >
                  + Add Tag
                </Button>
              </Stack>
            </Stack>
          )}
          {farmsSaving && (
            <Typography variant="subtitle1">Saving Farms</Typography>
          )}
          {!farmsSaving && (
            <Stack direction="row" spacing={1} alignItems="center">
              <Typography variant="subtitle1">Farms</Typography>
              <Stack direction="row" spacing={0.5}>
                {landmark.farms.map((farm, index) => (
                  <Chip
                    key={index}
                    label={farm}
                    color="secondary"
                    onDelete={() => removeFarm.mutate(farm)}
                  />
                ))}
                <TextField
                  label="Add Farm"
                  onChange={(event) => setNewFarm(event.currentTarget.value)}
                  size="small"
                  value={newFarm}
                />
                <Button
                  onClick={() => {
                    if (!newFarm) {
                      return;
                    }
                    addFarm.mutate(newFarm);
                  }}
                >
                  + Add Farm
                </Button>
              </Stack>
            </Stack>
          )}
          <LandmarkLinks
            worldId={worldId}
            landmarkLinks={landmark.links}
            landmarkId={landmarkId}
            allowEditing
          />
        </Stack>
      )}
    </Container>
  );
};
