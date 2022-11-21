docker-compose down -v;

if [ -d "data" ]
then
  echo "Deleting old graph data";
  rm -rf data/;
fi

docker-compose up;

