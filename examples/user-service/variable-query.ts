import { Injectable } from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';

@Injectable()
export class VariableQueryService {
    constructor(@InjectModel('users') private userModel: Model<any>) { }

    async findActiveUsers() {
        // This query is defined in a variable
        const query = { isActive: true, role: 'admin' };

        // And passed here
        return this.userModel.find(query).exec();
    }
}
