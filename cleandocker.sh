#!/bin/bash
# Supprimer tous les conteneurs arrêtés
stopped_containers=$(docker ps -a -q)
if [ -z "$stopped_containers" ]; then
    echo "Aucun conteneur arrêté à supprimer."
else
    docker rm $stopped_containers
    echo "Tous les conteneurs arrêtés ont été supprimés."
fi

# Supprimer toutes les images Docker
images=$(docker images -q)
if [ -z "$images" ]; then
    echo "Aucune image à supprimer."
else
    docker rmi $images
    echo "Toutes les images ont été supprimées."
fi
