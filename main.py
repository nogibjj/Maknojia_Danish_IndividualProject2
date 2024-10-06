from lib.transform_load import load
from lib.query import general_query, create_record, delete_record, update_record
import sys


# Main script entry point
def main():
    if len(sys.argv) < 2:
        print("Please specify an operation: load, query, create, update, delete.")
        sys.exit(1)  # Exit with an error code

    operation = sys.argv[1]

    if operation == "load":
        print("Transforming and loading data...")
        load()

    elif operation == "query":
        if len(sys.argv) == 3:
            query = sys.argv[2]
            print("Querying data")
            general_query(query)
        else:
            print("Insufficient arguments. Usage: python main.py query <SQL_QUERY>")
            sys.exit(1)  # Exit with an error code

    elif operation == "create":
        # Ensure enough arguments are passed to create a new record
        if len(sys.argv) == 9:
            rk = int(sys.argv[2])
            player_name = sys.argv[3]
            team = sys.argv[4]
            opp = sys.argv[5]
            matchup = sys.argv[6]
            start_sit = sys.argv[7]
            proj_fpts = float(sys.argv[8])
            print("Create Record")
            create_record(rk, player_name, team, opp, matchup, start_sit, proj_fpts)
        else:
            print(
                "Insufficient arguments. Usage: python main.py create <rk> <player_name> <team> <opp> <matchup> <start_sit> <proj_fpts>"
            )
            sys.exit(1)  # Exit with an error code

    elif operation == "update":
        # Ensure enough arguments are passed to update an existing record
        if len(sys.argv) == 10:
            record_id = int(sys.argv[2])
            rk = int(sys.argv[3])
            player_name = sys.argv[4]
            team = sys.argv[5]
            opp = sys.argv[6]
            matchup = sys.argv[7]
            start_sit = sys.argv[8]
            proj_fpts = float(sys.argv[9])
            print("Update Record")
            update_record(
                record_id, rk, player_name, team, opp, matchup, start_sit, proj_fpts
            )
        else:
            print(
                "Insufficient arguments. Usage: python main.py update <record_id> <rk> <player_name> <team> <opp> <matchup> <start_sit> <proj_fpts>"
            )
            sys.exit(1)  # Exit with an error code

    elif operation == "delete":
        # Ensure a record_id argument is passed for deletion
        if len(sys.argv) == 3:
            record_id = int(sys.argv[2])
            print("Delete Record")
            delete_record(record_id)
        else:
            print("Insufficient arguments. Usage: python main.py delete <record_id>")
            sys.exit(1)  # Exit with an error code

    else:
        print(
            "Unknown operation. Please use one of the following: load, query, create, update, delete."
        )
        sys.exit(1)  # Exit with an error code


if __name__ == "__main__":
    main()
