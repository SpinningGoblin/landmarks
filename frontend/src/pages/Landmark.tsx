import { FC, useCallback, useState } from "react";
import { useParams } from "react-router-dom";
import {
  useAddBiome,
  useAddFarm,
  useAddTag,
  useLandmark,
  useLandmarks,
  useRemoveBiome,
  useRemoveFarm,
  useRemoveTag,
} from "../hooks/landmarks";
import {
  Button,
  Chip,
  Container,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { Coordinate } from "../components/Coordinate";
import { useBiomes } from "../hooks/minecraft";

export const Landmark: FC = () => {
  const { worldId, landmarkId } = useParams();
  const [newBiome, setNewBiome] = useState("");
  const addBiomeCallback = useCallback(() => {
    setNewBiome("");
  }, [setNewBiome]);
  const [newFarm, setNewFarm] = useState("");
  const addFarmCallback = useCallback(() => {
    setNewFarm("");
  }, [setNewFarm]);

  const [newTag, setNewTag] = useState("");
  const addTagCallback = useCallback(() => {
    setNewTag("");
  }, [setNewTag]);

  const { biomes } = useBiomes();
  const { landmarks } = useLandmarks(worldId);
  const { landmark, isLoading } = useLandmark(landmarkId);
  const { removeFarm } = useRemoveFarm(landmarkId);
  const { addFarm } = useAddFarm(addFarmCallback, landmarkId);
  const { removeBiome } = useRemoveBiome(landmarkId);
  const { addBiome } = useAddBiome(addBiomeCallback, landmarkId);
  const { removeTag } = useRemoveTag(landmarkId);
  const { addTag } = useAddTag(addTagCallback, landmarkId);

  const farmsSaving = removeFarm.isLoading || addFarm.isLoading;
  const biomesSaving = removeBiome.isLoading || addBiome.isLoading;
  const tagsSaving = removeTag.isLoading || addTag.isLoading;

  console.log((landmarks ?? []).filter((l) => l.id !== landmarkId));

  return (
    <Container>
      {isLoading && <Typography>Loading ...</Typography>}
      {landmark && (
        <Stack spacing={4}>
          <Typography variant="h2">{landmark.metadata.name}</Typography>
          <Coordinate coordinate={landmark.metadata.coordinate} />
          <Typography variant="h6" textTransform="capitalize">
            Dimension: {landmark.dimension}
          </Typography>
          <Typography variant="body2" textTransform="capitalize">
            Notes: {landmark.metadata.notes}
          </Typography>
          {biomesSaving && (
            <Typography variant="subtitle1">Saving Biomes</Typography>
          )}
          {!biomesSaving && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Biomes</Typography>
              <Stack
                direction="row"
                spacing={0.5}
                alignItems="center"
                flexWrap="wrap"
              >
                {(landmark.biomes ?? []).map((biome, index) => (
                  <Chip
                    key={index}
                    label={biome.replaceAll("_", " ")}
                    color="primary"
                    onDelete={() => removeBiome.mutate(biome)}
                  />
                ))}
                <FormControl>
                  <InputLabel>Biomes</InputLabel>
                  <Select
                    label="Biomes"
                    value={newBiome}
                    onChange={(event) => setNewBiome(event.target.value)}
                    style={{ minWidth: "20em" }}
                    size="small"
                  >
                    {biomes?.map((b) => (
                      <MenuItem key={b} value={b}>
                        <Typography style={{ textTransform: "capitalize" }}>
                          {b.replaceAll("_", " ")}
                        </Typography>
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
                <Button
                  onClick={() => {
                    if (!newBiome) {
                      return;
                    }
                    addBiome.mutate(newBiome);
                  }}
                >
                  + Add Biome
                </Button>
              </Stack>
            </Stack>
          )}
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
        </Stack>
      )}
    </Container>
  );
};
