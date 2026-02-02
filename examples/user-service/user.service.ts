import { Injectable } from '@nestjs/common';
import { InjectConnection } from '@nestjs/mongoose';
import { Connection } from 'mongoose';

@Injectable()
export class UserService {
  constructor(@InjectConnection() private readonly connection: Connection) {}

  async findUserByEmail(email: string) {
    return this.connection.useDb('users').collection('users').findOne({
      email: email,
      isActive: true
    });
  }

  async findActiveUsers(limit: number = 10) {
    return this.connection.useDb('users').collection('users').find({
      isActive: true,
      lastLogin: { $exists: true }
    }).limit(limit).toArray();
  }

  async findUsersByRole(role: string) {
    return this.connection.useDb('users').collection('users').find({
      role: role,
      isActive: true
    }).toArray();
  }

  async aggregateUserStats() {
    return this.connection.useDb('users').collection('users').aggregate([
      { $match: { isActive: true } },
      { $group: { _id: '$role', count: { $sum: 1 } } }
    ]).toArray();
  }

  async countUsersByDepartment(department: string) {
    return this.connection.useDb('users').collection('users').countDocuments({
      department: department,
      isActive: true
    });
  }

  async searchUsers(query: string) {
    return this.connection.useDb('users').collection('users').find({
      $or: [
        { name: { $regex: query, $options: 'i' } },
        { email: { $regex: query, $options: 'i' } }
      ]
    }).toArray();
  }
}